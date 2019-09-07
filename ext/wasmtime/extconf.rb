# frozen_string_literal: true

unless system('cargo --version')
  abort 'Building native extention requires Rust (https://rustup.rs/).'
end
