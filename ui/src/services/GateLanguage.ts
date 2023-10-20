import hljs from 'highlight.js';

export function registerGateLanguage() {
  hljs.registerLanguage('gate', function () {
    return {
      case_insensitive: true, // language is case-insensitive
      // keywords: 'for if while',
      contains: [
        // {
        //   className: 'string',
        //   begin: '"',
        //   end: '"'
        // },
        {
          className: 'fixed',
          begin: /f_\d+/,
        },
        {
          className: 'advice',
          begin: /a_\d+/,
        },
        {
          className: 'hex',
          begin: /0x[0-9a-f]+/,
        },
        {
          className: 'next-rotation',
          begin: /\[\d+\]/,
        },
        {
          className: 'prev-rotation',
          begin: /\[-\d+\]/,
        },
        // hljs.COMMENT(
        //   '/\\*', // begin
        //   '\\*/', // end
        //   {
        //     contains: [{
        //       className: 'doc',
        //       begin: '@\\w+'
        //     }]
        //   }
        // )
      ],
    };
  });
}
