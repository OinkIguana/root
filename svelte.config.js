const preprocess = require('svelte-preprocess');

module.exports = {
  preprocess: preprocess({
    babel: true,
  }),
};