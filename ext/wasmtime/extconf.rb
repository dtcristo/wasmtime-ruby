# frozen_string_literal: true

if system('cargo --version')
  abort 'Building native extention requires Rust (https://rustup.rs/).'
end
