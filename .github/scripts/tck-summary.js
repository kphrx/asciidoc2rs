import fs from 'node:fs';
import readline from 'node:readline';

export async function parseOutputFile({path, core}) {
  const chunkStream = fs.createReadStream(path, {encoding: 'utf8'});
  const lineStream = readline.createInterface({
    input: chunkStream,
    crlfDelay: Infinity,
  });

  const summary = () => Object.create(Object.getPrototypeOf(core.summary)).emptyBuffer();

  const currentDescribes = [];
  const tests = [];
  const suites = [];
  const table = [[{data: 'Status', header: true}, {data: 'Test Case', header: true}, {data: 'Execution time', header: true}]];
  const commandErrors = [];
  let codeBuffer = '';
  let currentFailedResult = {};
  for await (const line of lineStream) {
    if (line.length === 0 && codeBuffer.length === 0) {
      continue;
    }

    const describePosition = line.indexOf('▶');
    if (describePosition !== -1) {
      const name = line.slice(describePosition + 2);
      currentDescribes.push(name);
      continue;
    }

    if (currentDescribes.length > 0) {
      const currentDescribe = currentDescribes.toReversed()[0];

      const endPassedDescribePosition = line.indexOf(`✔ ${currentDescribe} (`);
      const endFailedDescribePosition = line.indexOf(`✖ ${currentDescribe} (`);
      if (endPassedDescribePosition !== -1 || endFailedDescribePosition !== -1) {
        const time = line.slice(endPassedDescribePosition + endFailedDescribePosition + 5 + currentDescribe.length).replace(')', '');
        suites.push({status: endPassedDescribePosition !== -1, name: currentDescribes.join('.'), time});
        currentDescribes.pop();
        continue;
      }

      const passedItPosition = line.indexOf('✔');
      const failedItPosition = line.indexOf('✖');
      if (passedItPosition !== -1 || failedItPosition !== -1) {
        const [name, time] = line.slice(passedItPosition + failedItPosition + 3).replace(')', '').split(' (');
        tests.push({status: passedItPosition !== -1, suite: currentDescribes.join('.'), name, time});
        table.push([passedItPosition !== -1 ? 'Pass&nbsp;✅' : 'Fail&nbsp;❌', name, time]);
        continue;
      }

      continue;
    }

    if (line.startsWith('✖ failing tests:')) {
      const time = suites.find(suite => suite.name === 'tests').time;
      core.summary.addRaw(`Execution time: ${time}`, true);
      await core.summary.write();

      core.summary.addDetails('Test Cases', summary().addTable(table).stringify());
      await core.summary.write();
      continue;
    }

    if (line.startsWith('test at harness/lib/test-framework.js')) {
      continue;
    }

    const failedTestResultMarker = line.indexOf('✖');
    if (failedTestResultMarker !== -1) {
      if (codeBuffer.length > 0) {
        tests.find(test => {
          return test.name === currentFailedResult.name && test.suite === currentFailedResult.suite;
        }).detail = {
          message: currentFailedResult.message,
          diff: currentFailedResult.diff,
          info: codeBuffer,
        };
      } else if (currentFailedResult.error != null) {
        commandErrors.push({...currentFailedResult});
      }
      codeBuffer = '';
      currentFailedResult = {name: line.slice(2).split(' (')[0]};
      continue;
    }

    if (line.indexOf('AssertionError') !== -1) {
      const message = line.split(']: ')[1];
      const [msg, name] = message.split(' for ');
      currentFailedResult.suite = name.split('/').slice(0, -1).join('.');
      currentFailedResult.message = `${msg.trim()} for <code>${name.trim()}</code>`;
      continue;
    }

    const code = line.slice(2);
    if (currentFailedResult.suite == null) {
      currentFailedResult.error ??= '';
      currentFailedResult.error += `${code}\n`;
      continue;
    }
    if (code === '') {
      continue;
    }
    if (code === '+ actual - expected') {
      codeBuffer += '+ actual\n- expected\n';
      continue;
    }
    if (code === '] {') {
      if (codeBuffer.length > 0) {
        currentFailedResult.diff = codeBuffer;
      }
      codeBuffer = `{\n`;
      continue;
    }
    codeBuffer += `${code}\n`;
  }

  if (codeBuffer.length > 0) {
    tests.find(test => {
      return test.name === currentFailedResult.name && test.suite === currentFailedResult.suite;
    }).detail = {
      message: currentFailedResult.message,
      diff: currentFailedResult.diff,
      info: codeBuffer,
    };
  } else if (currentFailedResult.error != null) {
    commandErrors.push({...currentFailedResult});
  }
  codeBuffer = '';

  const suiteTrees = [];
  const createNestedList = (tree, tests, time, status) => {
    const name = tree.shift();
    if (tree.length === 0) {
      return [name, {tests, time, status}];
    }
    return [name, {children: [createNestedList(tree, tests, time, status)]}];
  };
  const createSuiteTree = (list, tree, tests, time, status) => {
    const current = list.toReversed()[0];
    if (current == null || current[0] !== tree[0]) {
      list.push(createNestedList(tree, tests, time, status));
      return;
    }
    if (tree.length === 1) {
      current[1].tests = tests;
      current[1].time = time;
      current[1].status = status;
      return;
    }
    createSuiteTree(current[1].children, tree.slice(1), tests, time, status);
  };
  for (const suite of suites) {
    const currentSuiteTests = tests.filter(test => test.suite === suite.name);
    createSuiteTree(suiteTrees, suite.name.split('.').slice(1), currentSuiteTests, suite.time, suite.status);
  }

  const createList = (children) => {
    return children.map(([name, info]) => {
      const list = summary().addRaw(`${info.status ? '✅' : '❌'} ${name}: ${info.time}`, true);
      if (info.children?.length > 0) {
        return list.addList(createList(info.children)).stringify();
      }

      const tableBuffers = [[
        {data: 'Status', header: true},
        {data: 'Test Case', header: true},
        {data: 'Execution Time', header: true}
      ]];
      if (!info.status) {
        tableBuffers[0].push({data: '', header: true});
      }
      for (const test of info.tests) {
        const tableBuffer = [test.status ? 'Pass&nbsp;✅' : 'Fail&nbsp;❌', test.name, test.time];
        if (!info.status) {
          tableBuffer.push(
            test.status ? ''
            : test.detail == null ? 'Failed Command'
            : summary()
            .addRaw(test.detail.message, true)
            .addDetails('Error Info', summary()
              .addCodeBlock(test.detail.diff, 'diff')
              .addCodeBlock(test.detail.info, 'js')
              .stringify())
            .stringify());
        };
        tableBuffers.push(tableBuffer);
      }
      list.addTable(tableBuffers);

      return list.stringify();
    });
  };
  for (const [section, suiteTree] of suiteTrees) {
    if (section == null) {
      continue;
    }
    core.summary.addHeading(section, 2).addList(createList(suiteTree.children));
    await core.summary.write();
  }
  if (commandErrors.length > 0) {
    core.summary.addHeading('command errors', 2);
    for (const commandError of commandErrors) {
      core.summary.addDetails(`❌ ${commandError.name}`,
        summary()
        .addCodeBlock(commandError.error)
        .stringify());
    }
    await core.summary.write();
  }
}
