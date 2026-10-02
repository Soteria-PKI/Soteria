module.exports = {
  extends: ['@commitlint/config-conventional'],

  rules: {
    'scope-enum': [
      2,
      'always',
      ['desktop', 'web-backend', 'web-frontend'],
    ],
    'jira-key': [2, 'always'],
  },

  plugins: [
    {
      rules: {
        'jira-key': ({ header }) => {
          const pattern = /^[a-z]+(?:([^)]+))?: SS-\d+ /;

          return [
            pattern.test(header),
            'commit message must contain a Jira key such as SS-[YOUR ISSUE KEY HERE]',
          ];
        },
      },
    },
  ],
};
