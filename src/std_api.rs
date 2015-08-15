// This file was derived from rust's own libstd/process.rs with the following
// copyright:
//
// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
use std::ffi::OsStr;
use std::default::Default;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::process::Stdio;

use ffi_util::ToCString;
use Command;

impl Command {
    /// Constructs a new `Command` for launching the program at
    /// path `program`, with the following default configuration:
    ///
    /// * No arguments to the program
    /// * Inherit the current process's environment
    /// * Inherit the current process's working directory
    /// * Inherit stdin/stdout/stderr for `spawn` or `status`, but create pipes for `output`
    ///
    /// Builder methods are provided to change these defaults and
    /// otherwise configure the process.
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        Command {
            filename: program.to_cstring(),
            args: vec![program.to_cstring()],
            environ: None,
            config: Default::default(),
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }

    /// Add an argument to pass to the program.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.args.push(arg.to_cstring());
        self
    }

    /// Add multiple arguments to pass to the program.
    pub fn args<S: AsRef<OsStr>>(&mut self, args: &[S]) -> &mut Command {
        self.args.extend(args.iter().map(ToCString::to_cstring));
        self
    }

    // TODO(tailhook) It's only public for our run module any better way?
    pub fn init_env_map(&mut self) {
        if self.environ.is_none() {
            self.environ = Some(env::vars_os().collect());
        }
    }

    /// Inserts or updates an environment variable mapping.
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Command
        where K: AsRef<OsStr>, V: AsRef<OsStr>
    {
        self.init_env_map();
        self.environ.as_mut().unwrap().insert(
            key.as_ref().to_os_string(),
            val.as_ref().to_os_string());
        self
    }

    /// Removes an environment variable mapping.
    pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Command {
        self.init_env_map();
        self.environ.as_mut().unwrap().remove(key.as_ref());
        self
    }

    /// Clears the entire environment map for the child process.
    pub fn env_clear(&mut self) -> &mut Command {
        self.environ = Some(HashMap::new());
        self
    }

    /// Sets the working directory for the child process.
    ///
    /// Note: in case of chroot or pivot root the working directory is set
    /// inside the new root.
    ///
    /// However, if you leave `current_dir` unspecified chroot will translate
    /// directory path, if possible or otherwise set root dir to new root.
    /// The pivot_root behaves same as chroot, i.e. it doesn't set current
    /// directory in `old_root`.
    ///
    /// At the end of the day, the ``cmd.current_dir(env::current_dir())`` is
    /// not no-op if using chroot/pivot_root.
    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Command
    {
        self.config.work_dir = Some(dir.as_ref().to_cstring());
        self
    }

    /// Configuration for the child process's stdin handle (file descriptor 0).
    pub fn stdin(&mut self, cfg: Stdio) -> &mut Command {
        self.stdin = Some(cfg);
        self
    }

    /// Configuration for the child process's stdout handle (file descriptor 1).
    pub fn stdout(&mut self, cfg: Stdio) -> &mut Command {
        self.stdout = Some(cfg);
        self
    }

    /// Configuration for the child process's stderr handle (file descriptor 2).
    pub fn stderr(&mut self, cfg: Stdio) -> &mut Command {
        self.stderr = Some(cfg);
        self
    }

}
