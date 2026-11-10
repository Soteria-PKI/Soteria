module.exports = {
  extends: ["./commitlint.base.config.js"],
  rules: {
    "jira-key": [2, "always"],
  },

  plugins: [
    {
      rules: {
        "jira-key": ({ header }) => {
          const pattern = /^[a-z]+(?:([^)]+))?: SS-\d+ /;

          return [
            pattern.test(header),
            "commit message must contain a Jira key such as SS-[YOUR ISSUE KEY HERE]",
          ];
        },
      },
    },
  ],
};
