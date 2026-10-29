const arg1 = Deno.args[0];
const decoder = new TextDecoder("utf8");
const jiraRegex = /^SS-\d{3,}(-([a-zA-Z])+)*$/;

const decode = (stdout: Uint8Array<ArrayBufferLike>): string => {
  return decoder.decode(stdout);
};

const getBranch = async () => {
  const command = new Deno.Command("git", {
    args: ["branch", "--show-current"],
  });
  const { stdout }: { stdout: Uint8Array } = await command.output();

  const branchName = decode(stdout).trim();
  return branchName;
};

const runCommand = async () => {
  const branch = await getBranch();
  let baseArguments =
    "run --allow-read --allow-sys --allow-run --allow-env npm:@commitlint/cli ";

  if (jiraRegex.test(branch)) {
    baseArguments = baseArguments.concat(`--edit ${arg1}`);
  } else {
    baseArguments = baseArguments.concat(
      `--config commitlint.base.config.js --edit ${arg1}`,
    );
  }
  const command = new Deno.Command("deno", { args: baseArguments.split(" ") });
  const { success, stdout } = await command.output();
  console.log(decode(stdout));

  if (!success) {
    Deno.exit(1);
  }
};

runCommand();
