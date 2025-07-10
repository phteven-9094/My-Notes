import re


# Time Validating
def is_valid_time(input):
    pattern = re.compile(r"^\d\d?:\d\d$")
    match = pattern.search(input)
    if match:
        return True
    return False


# Parsing Bytes
def parse_bytes(input):
    binary_regex = re.compile(r"\b[10]{8}\b")
    results = binary_regex.findall(input)
    return results


# Date Parsing
def parse_date(input):
    pattern = re.compile("^(\d\d)[,/.](\d\d)[,/.](\d{4})$")
    match = pattern.search(input)
    if match:
        return {
            "d": match.group(1),
            "m": match.group(2),
            "y": match.group(3),
        }
    return None

# Profanity Filter
 
def censor(input):
    pattern = re.compile(r'\bfrack\w*\b', re.IGNORECASE)
    return pattern.sub("CENSORED", input)