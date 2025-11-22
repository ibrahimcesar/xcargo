/**
 * Docusaurus plugin to add X-Clacks-Overhead header
 *
 * "A man is not dead while his name is still spoken."
 * - Terry Pratchett, Going Postal
 *
 * @see http://www.gnuterrypratchett.com/
 */

module.exports = function (context, options) {
  return {
    name: 'clacks-overhead-plugin',

    configureWebpack(config, isServer, utils) {
      return {
        plugins: [
          // Add custom header via Webpack DevServer configuration
          utils.getPlugin('webpack.DefinePlugin').map(plugin => {
            return plugin;
          }),
        ],
      };
    },

    injectHtmlTags({content}) {
      return {
        headTags: [
          {
            tagName: 'meta',
            attributes: {
              'http-equiv': 'X-Clacks-Overhead',
              content: 'GNU Terry Pratchett',
            },
          },
        ],
      };
    },
  };
};
