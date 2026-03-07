
/// 
/// 
/// # Arguments 
/// 
/// * `code`: 
/// 
/// returns: bool 
/// 
/// # Examples 
/// 
/// ```
/// 
/// ```
pub fn is_alphabetic_palindrome(code: &str) -> bool {
	let code = code
		.chars()
		.filter(|&c| c.is_alphabetic())
		.collect::<String>()
		.to_ascii_lowercase();

	let reverse = code
		.chars()
		.rev()
		.collect::<String>();

	code == reverse
}

#[cfg(test)]
mod tests {
	use super::*;
#[test]


	fn rotate_test() {
		let data1 = [("aAaAaAaAaAaAaaaa",true),("aAaÀÁÂÃÄÅàáâãäåɑΑαаᎪＡａdef£$%%*&^&*fed/ａＡᎪаαΑɑåäãâáàÅÄÃÂÁÀaAa",true),("aAaÀÁÂÃÄÅàáâãäåɑΑαаᎪＡａdef£$%%*&^&*fed/ａＡᎪаαΑɑåäãâáàÅÄÃÂÁÀaAa",true),(":։܃܄∶꞉：：꞉∶܄܃։:",true),("BbßʙΒβВЬᏴᛒＢｂｂＢᛒᏴЬВβΒʙßbB",true),("CcϲϹСсᏟⅭⅽ𐐠𺀠ＣｃｃＣ�𺀠�ⅽⅭᏟсСϹϲcCCcϲϹСсᏟⅭⅽ���ＣｃｃＣ�𺀠�ⅭᏟсСϹϲcC",false),("DdĎďĐđԁժᎠḍⅮⅾＤｄｄＤⅾⅮḍᎠժԁđĐďĎdD",true),("EeÈÉÊËéêëĒēĔĕĖėĘĚěΕЕеᎬＥｅｅＥᎬеЕΕěĚĘėĖĕĔēĒëêéËÊÉÈeE",true),];
		let data2 = [("abc*def<>?fed+cba",true),("3D565A79&9kgFX6x4£$%%*&^&*4x6XFgk9/97A565D3",true),("4F1ED380'zdSigBg9£$%%*&^&*9gBgiSdz,083DE1F4",true),("FE77904F,TDLWLVeV<>?VeVLWLDT!F40977EF",true),("EBC49008!uRZqv1gf<>?fg1vqZRu%80094CBE",true),("94D0C00C)0RNbOJly,./$ylJObNR0,C00C0D49",true),("5C196FA5'PPc3mXVM}{P@:?><,./#'!£$%^&*()VXm3cPP+5AF691C5",false),("260B6B54$HwC5YcSJ<>?JScY5CwH+45B6B062",true),];
		let data3 = [("6xczvB4/\"8D942)7IGiRC9m6B48D942)7Iwty  GiRC9m249D84B6",false),("AE500zxvcF35/XSVGbFK6AE500F35/wyXSVGb FK653F005EA",false),("E83C7FE2'zxcv56tHeJu2E83C7FEeu2'56tHeJu22EF7C38E",false),("B55C5E6E(fS9nzxcvjHsJB55ljghC5E6E(fS9njHsJE6E5C55B",false),("2F8770D1!oxIhEjr6eryuzxcv2F8770D1!oxIhEjr61D0778F2",false),("B007C751*bL1qhjlkaOtdB007xzcvC751*bL1qaOtd157C700B",false),("957417ED+YFeyqJvikv95sdf7417EDxzcv+YFqJvikvDE714759",false),("45A8A7Aadsf9#92QVoc6u45A8A7A9#92QzxvcVoc6u9A7A8A54",false),];
		for data in [data1,data2,data3]{
			_data_runner(data);}
	}

	fn _data_runner(data: [(&str, bool); 8]) {
		for d in data.iter() {
			assert_eq!(is_alphabetic_palindrome(d.0), d.1);
		} } }
