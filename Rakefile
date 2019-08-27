require 'bundler/setup'
require 'rake/testtask'
import 'lib/tasks/helix_runtime.rake'

Rake::TestTask.new(:test) do |t|
  t.libs << 'test'
  t.test_files = FileList['test/**/*_test.rb']
end

task test: :build
task default: :test
