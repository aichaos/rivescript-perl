RiveScript version 1.22
=======================

INSTALLATION

To install this module type the following:

  perl Makefile.PL
  make
  make test
  make install

RPM BUILD

To build a RedHat package file for installing RiveScript, use the
rpmbuild Perl script provided in the subversion repository.

Usage: perl rpmbuild

This results in a slightly different RPM than what you'd get via
cpan2rpm or cpan2dist... along with installing the module in its
proper place in your Perl libs, it will also install the `rivescript`
and `rsup` utilities from the bin/ folder into your /usr/bin
directory.

DEPENDENCIES

Nothing that isn't standard.

COPYRIGHT AND LICENSE

The Perl RiveScript interpreter is dual licensed as of version 1.22. For open
source applications the module is using the GNU General Public License. If
you'd like to use the RiveScript module in a closed source or commercial
application, contact the author for more information.

  RiveScript - Rendering Intelligence Very Easily
  Copyright (C) 2011 Noah Petherbridge

  This program is free software; you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation; either version 2 of the License, or
  (at your option) any later version.

  This program is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program; if not, write to the Free Software
  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
