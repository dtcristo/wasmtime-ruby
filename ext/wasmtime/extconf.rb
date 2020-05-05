# frozen_string_literal: true

unless system('rustup --version')
  abort 'Building native extention requires Rust with rustup (https://rustup.rs/).'
end
