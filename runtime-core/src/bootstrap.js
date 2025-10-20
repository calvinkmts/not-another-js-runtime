const { core } = Deno;

function argsToMessage(...args) {
  return args.map((arg) => JSON.stringify(arg)).join(" ");
}

globalThis.console = {
  log: (...args) => {
    core.print(`[out]: ${argsToMessage(...args)}\n`, false);
  },
  error: (...args) => {
    core.print(`[err]: ${argsToMessage(...args)}\n`, true);
  },
};

globalThis.fs = {
  readFile: (path) => {
    return core.ops.op_read_file(path);
  },
  writeFile: (path, content) => {
    return core.ops.op_write_file(path, content);
  },
  removeFile: (path) => {
    return core.ops.op_remove_file(path);
  },
};
