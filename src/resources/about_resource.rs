use futures::{self, Future};
use hyper;
use hyper::header::ContentType;
use hyper::server::*;

use mimes::*;
use site::Layout;
use web::{Resource, ResponseFuture};

pub struct AboutResource;

impl AboutResource {
    pub fn new() -> Self {
        AboutResource
    }
}

enum License {
    Bsd3Clause,
    Gpl3,
    Mit,
    Mpl2,
    Ofl11,
}

impl License {
    fn link(&self) -> &'static str {
        use self::License::*;
        match self {
            &Bsd3Clause => "bsd-3-clause",
            &Gpl3 => "gpl3",
            &Mit => "mit",
            &Mpl2 => "mpl2",
            &Ofl11 => "sil-ofl-1.1",
        }
    }

    fn name(&self) -> &'static str {
        use self::License::*;
        match self {
            &Bsd3Clause => "BSD-3-Clause",
            &Gpl3 => "GPL3",
            &Mit => "MIT",
            &Mpl2 => "MPL2",
            &Ofl11 => "OFL-1.1",
        }
    }
}

struct Dependency {
    name: &'static str,
    copyright: &'static str,
    license: License,
}

lazy_static! {
    static ref DEPS: &'static [Dependency] = &[
        Dependency {
            name: "Amatic SC",
            copyright: "Copyright 2015 The Amatic SC Project Authors (contact@sansoxygen.com)",
            license: License::Ofl11,
        },
    ];
}

#[derive(BartDisplay)]
#[template="templates/about.html"]
struct Template<'a> {
    deps: &'a [Dependency]
}

impl<'a> Template<'a> {
    fn pkg_version(&self) -> &str { env!("CARGO_PKG_VERSION") }
}

impl Resource for AboutResource {
    fn allow(&self) -> Vec<hyper::Method> {
        use hyper::Method::*;
        vec![Options, Head, Get]
    }

    fn head(&self) -> ResponseFuture {
        Box::new(futures::finished(Response::new()
            .with_status(hyper::StatusCode::Ok)
            .with_header(ContentType(TEXT_HTML.clone()))
        ))
    }

    fn get(self: Box<Self>) -> ResponseFuture {
        let head = self.head();

        Box::new(head
            .and_then(move |head| {
                Ok(head
                    .with_body(Layout {
                        base: None, // Hmm, should perhaps accept `base` as argument
                        title: "About Sausagewiki",
                        body: &Template {
                            deps: &*DEPS
                        },
                    }.to_string()))
            }))
    }
}
