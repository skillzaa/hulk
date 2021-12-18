
pub fn gen_site_dir_struct()->Result<Vec<String>,Error>{
    let data_struct = self.data_dir_struct()?;
    let output:Vec<String> = data_struct.into_iter()
    .map(|i|i.replacen(
      &self.data_dir_name, 
      &self.site_dir_name,
      1))
      .collect::<Vec<String>>();
    Ok(output)
  }