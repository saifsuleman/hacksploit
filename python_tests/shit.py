from base64 import b64decode

USERNAME_TEST = "MEoEEPgAAAAAAAAAAAAAAAAAAAEwFAYIKoZIhvcNAwcECEABtbofCyjGBCCv0uPuFW7yZFeXZMMttC6b18G1P6o97nVe+VMycAieRQ=="
PASSWORD_TEST = "MDoEEPgAAAAAAAAAAAAAAAAAAAEwFAYIKoZIhvcNAwcECF7DKv38R6nvBBB/pKlHhoMNo4AI/0Uoxmvg"


def decode(data64):
    data = b64decode(data64)
    inp = self.SECItem(0, data, len(data))
    out = self.SECItem(0, None, 0)

    print(inp, out)


decode(PASSWORD_TEST)
