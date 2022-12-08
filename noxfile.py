import nox

nox.options.sessions = ["test"]


@nox.session
def test(session):
    session.install("-rrequirements-dev.txt")
    session.install("maturin")
    session.run_always("maturin", "develop", "-r")
    session.run("pytest")
