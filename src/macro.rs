#[macro_export]
macro_rules! def {
  (
    $fn:ident ( $($name:ident : $type:ty ),*) -> $result:ty $body:block
  )=>{
#[napi]
    pub async fn $fn($($name : $type),*) -> napi::Result<$result> {
      Ok({
        async move {
          Ok($body)
        }
        .await as anyhow::Result<_, anyhow::Error>
      }?)
    }
  };
}

#[macro_export]
macro_rules! napiImpl {
  (
    $cls:ty :
    $(
      $fn:ident (&$($name:ident $(: $type:ty)? ),*) -> $result:ty $body:block
    )*
  )=>{
    #[napi]
    impl $cls {
      $(
        #[napi]
        pub async fn $fn(&$($name$(: $type )?,)*) -> napi::Result<$result> {
          Ok(
            async move {
              Ok::<_,anyhow::Error>($body.to())
            }
            .await?
          )
        }
      )*
    }
  }
}
