use chrono::prelude::*;
use std::fs;
use std::{env, path::Path};

pub trait License: Sized {
    fn string(self) -> String;
}

pub struct MIT {
    pub name: String,
    pub year: String,
    pub text: String,
}

impl MIT {
    pub fn new(name: Option<String>, year: Option<String>) -> Self {
        let name = match name {
            Some(val) => val,
            None => match env::var("USER") {
                Ok(v) => v,
                Err(_) => String::from(""),
            },
        };
        let year = match year {
            Some(y) => y,
            None => Utc::now().year().to_string(),
        };

        let text: String = format!("Copyright {year} {name}

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the \"Software\"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
        
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
        
THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");

        Self { name, year, text }
    }
}

impl License for MIT {
    fn string(self) -> String {
        return self.text;
    }
}

pub struct Apache2 {
    pub name: String,
    pub year: String,
    pub text: String,
}

impl Apache2 {
    pub fn new(name: Option<String>, year: Option<String>) -> Self {
        let name = match name {
            Some(val) => val,
            None => match env::var("USER") {
                Ok(v) => v,
                Err(_) => String::from(""),
            },
        };
        let year = match year {
            Some(y) => y,
            None => Utc::now().year().to_string(),
        };

        let text: String = format!(
            "Copyright {year} {name}

Licensed under the Apache License, Version 2.0 (the \"License\");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an \"AS IS\" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License."
        );

        Self { name, year, text }
    }
}

impl License for Apache2 {
    fn string(self) -> String {
        return self.text;
    }
}

pub struct BSD3 {
    pub name: String,
    pub year: String,
    pub text: String,
}

impl BSD3 {
    pub fn new(name: Option<String>, year: Option<String>) -> Self {
        let name = match name {
            Some(val) => val,
            None => match env::var("USER") {
                Ok(v) => v,
                Err(_) => String::from(""),
            },
        };
        let year = match year {
            Some(y) => y,
            None => Utc::now().year().to_string(),
        };

        let text: String = format!("Copyright {year} {name}

Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
        );

        Self { name, year, text }
    }
}

impl License for BSD3 {
    fn string(self) -> String {
        return self.text;
    }
}

pub struct BSD2 {
    pub name: String,
    pub year: String,
    pub text: String,
}

impl BSD2 {
    pub fn new(name: Option<String>, year: Option<String>) -> Self {
        let name = match name {
            Some(val) => val,
            None => match env::var("USER") {
                Ok(v) => v,
                Err(_) => String::from(""),
            },
        };
        let year = match year {
            Some(y) => y,
            None => Utc::now().year().to_string(),
        };

        let text: String = format!("Copyright {year} {name}

        Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:

        1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
        
        2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
        
        THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS \"AS IS\" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."        
        );

        Self { name, year, text }
    }
}

impl License for BSD2 {
    fn string(self) -> String {
        return self.text;
    }
}


pub fn write_out<P: AsRef<Path>>(
    license: impl License,
    infile: P,
    outfile: P,
    prefix: Option<String>,
    suffix: Option<String>,
) -> std::io::Result<()> {
    let mut out = format!("");
    let content = fs::read_to_string(infile)?;
    let prefix = match prefix {
        Some(p) => p,
        None => "".to_owned(),
    };

    let suffix = match suffix {
        Some(s) => s,
        None => "".to_owned(),
    };

    for line in license.string().lines() {
        if !line.is_empty() {
            out.push_str(&prefix);
        }
        out.push_str(line);
        if !line.is_empty() {
            out.push_str(&suffix);
        }
        out.push_str("\n");
    }

    out.push_str(&content);

    fs::write(outfile, out)?;
    Ok(())
}
