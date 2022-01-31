# frozen_string_literal: true

RSpec.describe Ruffle do
  it "has a version number" do
    expect(Ruffle::VERSION).not_to be nil
  end

  it "has a #shuffle method" do
    expect(Ruffle).to respond_to(:shuffle)
  end
end
