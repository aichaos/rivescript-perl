# INSTALLATION

To install this module type the following:

```bash
perl Makefile.PL
make
make test
make install
```

# RPM BUILD

To build a RedHat package file for installing RiveScript, use the
rpmbuild Perl script provided in the subversion repository.

	Usage: perl rpmbuild

This results in a slightly different RPM than what you'd get via
cpan2rpm or cpan2dist... along with installing the module in its
proper place in your Perl libs, it will also install the `rivescript`
and `rsup` utilities from the bin/ folder into your /usr/bin
directory.

# DEPENDENCIES

Requires:

* [JSON](http://search.cpan.org/perldoc?JSON)

Recommends:

* [Clone](http://search.cpan.org/perldoc?Clone)
