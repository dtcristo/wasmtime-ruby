# frozen_string_literal: true

require 'bundler/setup'
require 'rspec/core/rake_task'

import 'lib/tasks/build.rake'

desc 'Format sources'
task :format do
  sh 'cargo fmt'
  sh 'bundle',
     'exec',
     'rbprettier',
     '--write',
     '**/*.{rb,rake,gemspec}',
     '**/{Rakefile,Gemfile}'
end

RSpec::Core::RakeTask.new(:spec)

task spec: :build
task default: :spec
