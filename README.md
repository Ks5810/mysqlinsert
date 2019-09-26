<h1 align="center">mysqlinsert</h1>
<br>
<p align="center"><strong>Description</strong>
<p>&nbsp; &nbsp; &nbsp; Mysql has a command called <i>mysqlimport</i> which
 locally reads a text file, and stores it to existing table. However,
logging into the server and creating a table, and then typing a long 
command like <i>load data infile...</i> can be painful.
<br>
<br>
&nbsp; &nbsp; &nbsp; This command line app reads a comma separated values (for now), 
creates table, and stores values into a table in mysql server. 
It only needs a file which contains type names corresponding to 
fields of the data. For example, if cars.txt has three fields, <i>"id, speed, dist</i>" 
the type specifier file should only contain a line "<i>int,int,int</i>".

<h2>Usage</h2>

1. Please download or clone this repository.
2. To build this app, rust and cargo must be installed.
They can be installed from [here](https://www.rust-lang.org/tools/install).

2. Once rust is installed, cd into the project root, and run the configuration
script:  <pre> chmod  +x  configure </pre>
         <pre> ./configure  <*host*>  <*user*>  <*database*></pre>
         where *hostname* is the server where the data is (only localhost is supported for now), *username* is your mysql
         username, and *database* is the target database name to create a table. 
3. If the configuration is successful, run this command in the project root:
        <pre> cargo  build  --release</pre> 
4. Install the app by typing: 
        <pre> ./install  <*install prefix*> </pre>
        If installation path doesn't exist or is empty, it will be installed
        to *$HOME/.mysqlinsert*

   **NOTE:** If a prefix is supplied, this script will create a directory,
*$PREFIX/mysqlinsert*
5. By logging out and in to terminal, the command should now be available.
6. Run mysqlinsert: 
       <pre> mysqlinsert  <*path to data*>  <*path to type specifier*></pre>
       If username is associated to a password, this app should prompt to enter
       the password   
7. To uninstall, run the uninstall script generated by installation:
       <pre>./uninstall</pre>
       If this results an error, please delete the installation directory,
       and check very last lines of .bashrc or .profile, and 
       remove the line with the prefix path.  