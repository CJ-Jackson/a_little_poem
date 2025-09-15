# Validation
validate-cannot-be-empty = Cannot be empty

validate-min-length =
    Must be at least { $min ->
        [one] 1 character
        *[other] { $min } characters
    }
validate-max-length =
    Must be at most { $max ->
        [one] 1 character
        *[other] { $max } characters
    }

validate-must-have-special-chars = Must contain at least one special character
validate-must-have-uppercase-and-lowercase = Must contain at least one uppercase and lowercase letter
validate-must-have-uppercase = Must contain at least one uppercase letter
validate-must-have-lowercase = Must contain at least one lowercase letter
validate-must-have-digit = Must contain at least one digit

validate-password-does-not-match = Does not match
validate-username-taken = Already taken

validate-password-entropy = Password entropy score must be over { $min }, try using a password manager?