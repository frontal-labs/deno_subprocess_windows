#[cfg(windows)]
mod windows_tests {
  use deno_subprocess_windows::Command;
  use deno_subprocess_windows::Stdio;

  #[test]
  fn command_builder_preserves_program_arguments_and_directory() {
    let mut command = Command::new("cmd.exe");
    command.arg("/C").arg("echo ok").current_dir("C:\\");
    assert_eq!(command.get_program().to_string_lossy(), "cmd.exe");
    assert_eq!(
      command
        .get_args()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<_>>(),
      vec!["/C", "echo ok"]
    );
    assert_eq!(command.get_current_dir().unwrap().to_string_lossy(), "C:\\");
  }

  #[test]
  fn stdio_constructors_cover_supported_modes() {
    let _ = Stdio::inherit();
    let _ = Stdio::piped();
    let _ = Stdio::null();
  }
}

#[cfg(not(windows))]
#[test]
fn windows_process_api_is_platform_gated() {
  assert!(!cfg!(windows));
}
