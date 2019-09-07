# frozen_string_literal: true

require 'bundler/setup'
require 'rake/testtask'

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

Rake::TestTask.new(test: :build) do |t|
  t.libs << 'test'
  t.test_files = FileList['test/**/*_test.rb']
end

task default: :test
