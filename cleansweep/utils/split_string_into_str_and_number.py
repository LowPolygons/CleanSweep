import re

# A bit tacky, but returns a (str, num) or None
def extract_number_from_end_of_string(item: str):
    maybe_number = re.search(r'\d+(\.\d+)?$', item)

    if not maybe_number:
        return None

    str_section = re.sub(r'\d+(\.\d+)?$', '', item)

    return (str_section, maybe_number.group())
