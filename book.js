const pkg = require('./package.json');
const pkgLock = require('./package-lock.json');

module.exports = {
  root: './docs',
  title: 'asciidoc2rs',
  description: pkg.description,
  gitbook: pkgLock.packages['node_modules/honkit'].version,
  honkit: pkgLock.packages['node_modules/honkit'].version,
};
