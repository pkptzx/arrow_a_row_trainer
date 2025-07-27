use forward_dll::forward_dll;

fn main() {
  forward_dll("C:\\Windows\\System32\\xinput1_3.dll").unwrap();
}