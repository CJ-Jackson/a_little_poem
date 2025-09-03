# Validation
validate-cannot-be-empty = Ne peut pas être vide

validate-min-length =
    Doit comporter au moins { $min ->
        [one] 1 caractère
        *[other] { $min } caractères
    }
validate-max-length =
    Doit comporter au maximum { $max ->
        [one] 1 caractère
        *[other] { $max } caractères
    }

validate-must-have-special-chars = Doit contenir au moins un caractère spécial
validate-must-have-uppercase-and-lowercase = Doit contenir au moins une lettre majuscule et une lettre minuscule.
validate-must-have-uppercase = Doit contenir au moins une lettre majuscule
validate-must-have-lowercase = Doit contenir au moins une lettre minuscule
validate-must-have-digit = MDoit contenir au moins un chiffre

validate-password-does-not-match = Ne correspond pas
validate-username-taken = Déjà pris