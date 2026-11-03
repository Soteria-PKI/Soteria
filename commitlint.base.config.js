module.exports = {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "scope-enum": [2, "always", ["desktop", "web-backend", "web-frontend"]],
    "subject-case": [0],
  },
};
