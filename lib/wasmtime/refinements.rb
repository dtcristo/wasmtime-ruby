module Wasmtime
  module Refinements
    refine String do
      def camelize(uppercase_first_letter = true)
        string = self
        if uppercase_first_letter
          string = string.sub(/^[a-z\d]*/, &:capitalize)
        else
          string = string.sub(/^(?:(?=\b|[A-Z_])|\w)/, &:downcase)
        end
        string.gsub(%r{(?:_|(\/))([a-z\d]*)}) { "#{$1}#{$2.capitalize}" }.gsub(
          '/',
          '::'
        )
      end
    end
  end
end
