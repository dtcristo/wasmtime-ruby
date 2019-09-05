module Wasmtime
  module Refinements
    refine String do
      def camelize(uppercase_first_letter = true)
        string = self
        if uppercase_first_letter
          string = string.sub(/^[a-z\d]*/) { |match| match.capitalize }
        else
          string = string.sub(/^(?:(?=\b|[A-Z_])|\w)/) { |match| match.downcase }
        end
        string.gsub(/(?:_|(\/))([a-z\d]*)/) { "#{$1}#{$2.capitalize}" }.gsub("/", "::")
      end
    end
  end
end
