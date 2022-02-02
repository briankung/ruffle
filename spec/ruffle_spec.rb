# frozen_string_literal: true

RSpec.describe Ruffle do
  it "has a #shuffle method" do
    expect(Ruffle).to respond_to(:shuffle)
  end

  it "shuffles the characters in a string" do
    string = "Ruffle"
    expect(Ruffle.shuffle(string)).to_not eq(string)
    expect(Ruffle.shuffle(string)).to_not eq(string.reverse)
  end
end
