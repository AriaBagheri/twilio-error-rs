from typing import List, Dict
from pathlib import Path

import requests

def main():
    url = "https://www.twilio.com/docs/api/errors/twilio-error-codes.json"

    response = requests.get(url)
    response.raise_for_status()
    data = response.json()

    products: Dict[str, List[object]] = {}
    for item in data:
        product = item.get("product") or "Miscellaneous"
        items = products.get(product)
        if items:
            items.append(item)
        else:
            products[product] = [item]
    product_names_raw = products.keys()
    product_names = list(map(lambda x: "".join(map(lambda m: m.capitalize(), x.split(" "))), products.keys()))
    module_names = map(lambda x: x.lower().replace(" ", "_"), products.keys())
    import_modules = "\n\n".join(map(lambda x: f"mod {x};\nuse {x}::*;", module_names))
    inner_enum = ",\n            ".join(map(lambda x: f"TwilioProductError::{x}(v) => Box::new(v) as Box<dyn SomeTwilioProductError>", product_names))
    main_enum = ",\n    ".join(map(lambda x: f"{x[0]}(Twilio{x[0]}Error)", zip(product_names, product_names_raw)))
    from_code_enum = list(map(lambda x: f".or_else(|| Twilio{x}Error::from_code(code).map(Into::into))", product_names))
    from_code_enum = "\n            ".join(from_code_enum)
    Path("src/products").mkdir(parents=True, exist_ok=True)
    with open("src/products/from_code.rs", "w") as f:
        f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use standard_error::traits::StandardErrorFromCodeTrait;
use crate::TwilioProductError;
use super::*;

impl StandardErrorFromCodeTrait for TwilioProductError {{
    fn from_code(code: usize) -> Option<Self>
    where
        Self: Sized
    {{
        None
            {from_code_enum}
    }}
}}
""")
    with open("src/products/mod.rs", "w") as f:
        f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
{import_modules}
mod from_code;
pub use from_code::*;

use serde::{{Deserialize, Deserializer, Serialize, Serializer}};
use serde::de::Error;
use standard_error::traits::StandardErrorFromCodeTrait;

#[derive(Debug, Clone)]
pub enum TwilioProductError {{
    {main_enum}
}}

use standard_error::traits::*;
pub trait SomeTwilioProductError: StandardErrorFromCodeTrait + StandardErrorCodeTrait + StandardErrorDescriptionTrait + StandardErrorCausesTrait + StandardErrorDocsTrait + StandardErrorMessageTrait + StandardErrorSolutionsTrait {{}}

impl TwilioProductError {{
    pub fn inner(&self) -> Box<dyn SomeTwilioProductError> {{
        self.clone().into_inner()
    }}
    pub fn into_inner(
        self,
    ) -> Box<dyn SomeTwilioProductError> {{
        match self {{
            {inner_enum}
        }}
    }}
}}

impl<'de> Deserialize<'de> for TwilioProductError {{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {{
        Ok(
            usize::deserialize(deserializer)
                .map(TwilioProductError::from_code)?
                .ok_or_else(|| Error::custom("Invalid error code!"))?
        )
    }}
}}

impl Serialize for TwilioProductError {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {{
        usize::serialize(&self.inner().code(), serializer)
    }}
}}
""")

    for product, items in products.items():
        folder = f"src/products/{product.lower().replace(" ", "_")}"
        class_name_raw = "".join(map(lambda x: x.capitalize(), product.split(" ")))
        class_name = f"Twilio{class_name_raw}Error"
        Path(folder).mkdir(parents=True, exist_ok=True)

        enum_str = ",\n    ".join(map(lambda x: f"ErrorCode{x["code"]}", items))
        messages = ",\n            ".join(map(lambda x: f"{class_name}::ErrorCode{x["code"]} => r#\"{x["message"]}\"#.into()", items))
        docs = ",\n            ".join(map(lambda x: f"{class_name}::ErrorCode{x["code"]} => r#\"{x["docs"]}\"#", items))
        causes = ",\n            ".join(map(lambda x: f"{class_name}::ErrorCode{x["code"]} => {f"Some(r#\"{x["causes"]}\"#)" if x["causes"] else None}", items))
        solutions = ",\n            ".join(map(lambda x: f"{class_name}::ErrorCode{x["code"]} => {f"Some(r#\"{x["solutions"].replace("\r", "")}\"#)" if x["solutions"] else None}", items))
        description = ",\n            ".join(map(lambda x: f"{class_name}::ErrorCode{x["code"]} => {f"Some(r#\"{x["description"]}\"#)" if x["description"] else None}", items))
        code = ",\n            ".join(map(lambda x: f"{class_name}::ErrorCode{x["code"]} => {x["code"]}", items))
        from_code = ",\n            ".join(map(lambda x: f"{x["code"]} => Some({class_name}::ErrorCode{x["code"]})", items))
        with open(f"{folder}/mod.rs", "w") as f:
            f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
mod message;
mod docs;
mod causes;
mod solutions;
mod description;
mod code;
mod from_code;

pub use message::*;
pub use docs::*;
pub use causes::*;
pub use solutions::*;
pub use description::*;
pub use code::*;
pub use from_code::*;

use serde::{{Deserialize, Deserializer, Serialize, Serializer}};
use serde::de::Error;
use standard_error::traits::{{StandardErrorCodeTrait, StandardErrorFromCodeTrait}};

#[derive(Debug, Clone)]
pub enum {class_name} {{
    {enum_str}
}}


impl<'de> Deserialize<'de> for {class_name} {{
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {{
            Ok(
                usize::deserialize(deserializer)
            .map({class_name}::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }}
}}

impl Serialize for {class_name} {{
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {{
            usize::serialize(&self.code(), serializer)
    }}
}}

use crate::TwilioProductError;
impl Into<TwilioProductError> for {class_name} {{
    fn into(self) -> TwilioProductError {{
        TwilioProductError::{class_name_raw}(self)
    }}
}}

impl crate::products::SomeTwilioProductError for {class_name} {{}}

""")
        with open(f"{folder}/message.rs", "w") as f:
            f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::{class_name};
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for {class_name} {{
    fn message(&self) -> Cow<'static, str> {{
        match self {{
            {messages}
        }}
    }}
}}
""")
        with open(f"{folder}/docs.rs", "w") as f:
            f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::{class_name};
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for {class_name} {{
    fn docs(&self) -> &'static str {{
        match self {{
            {docs}
        }}
    }}
}}
""")
        with open(f"{folder}/causes.rs", "w") as f:
            f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::{class_name};
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for {class_name} {{
    fn causes(&self) -> Option<&'static str> {{
        match self {{
            {causes}
        }}
    }}
}}
""")
        with open(f"{folder}/solutions.rs", "w") as f:
            f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::{class_name};
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for {class_name} {{
    fn solutions(&self) -> Option<&'static str> {{
        match self {{
            {solutions}
        }}
    }}
}}
""")
        with open(f"{folder}/description.rs", "w") as f:
            f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::{class_name};
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for {class_name} {{
    fn description(&self) -> Option<&'static str> {{
        match self {{
            {description}
        }}
    }}
}}
""")
        with open(f"{folder}/code.rs", "w") as f:
            f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::{class_name};
use standard_error::traits::StandardErrorCodeTrait;

impl StandardErrorCodeTrait for {class_name} {{
    fn code(&self) -> usize {{
        match self {{
            {code}
        }}
    }}
}}
""")
        with open(f"{folder}/from_code.rs", "w") as f:
            f.write(f"""// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::{class_name};
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for {class_name} {{
    fn from_code(code: usize) -> Option<Self> {{
        match code {{
            {from_code},
            _ => None
        }}
    }}
}}
""")

    # print("Products found:")
    # print(json.dumps(products["Notify"], indent=2))

if __name__ == "__main__":
    main()
