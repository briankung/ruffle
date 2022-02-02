# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

task default: :spec

desc "Compile the ruffle crate"
task :compile do
  cargo_builder_gem = [
    "ruby",
    "-I#{ENV["RUBYGEMS_PATH"]}/lib",
    "#{ENV["RUBYGEMS_PATH"]}/bin/gem"
  ]
  gemspec = File.expand_path('ruffle.gemspec')
  output = File.expand_path('ruffle.gem')

  system *cargo_builder_gem, "list", "-i", "^ruffle$"
  gem_installed = Process.last_status.success?

  system *cargo_builder_gem, 'uninstall', 'ruffle' if gem_installed
  system *cargo_builder_gem, 'build', gemspec, '--output', output
  system *cargo_builder_gem, 'install', output
end

# Rake::Task[:spec].prerequisites << :compile
