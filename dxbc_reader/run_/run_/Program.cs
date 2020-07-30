using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.IO;
using System.Diagnostics; 

namespace run_
{
    class Program
    {
        static void Main(string[] args)
        {
            //    string configfile = Path.GetDirectoryName(JsonPathexe);
            Process proc = new Process();
            string targetDir = (Directory.GetCurrentDirectory() + "\\build_ps.bat");
            Console.Write(targetDir);
            //proc.StartInfo.WorkingDirectory = targetDir;
            //proc.StartInfo.FileName = "";
            // proc.StartInfo.Arguments = string.Format("10");
            //proc.StartInfo.UseShellExecute = true;
            Process.Start(targetDir);
            // proc.WaitForExit();
        }
    }
}
