def dispatch(label):
    return label in ("trusted", "maintainer")

def audit_note():
    return "A_AUDIT_ONLY"

def measure_note():
    return "B_METRIC_ONLY"
