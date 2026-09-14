#[doc = "Register `OPTION_EO_BYTES` reader"]
pub type R = crate::R<OptionEoBytesSpec>;
#[doc = "Field `EXE_ONLY1_START` reader - Exe only 1 area start offset"]
pub type ExeOnly1StartR = crate::FieldReader;
#[doc = "Field `EXE_ONLY1_END` reader - Exe only 1 area end offset"]
pub type ExeOnly1EndR = crate::FieldReader;
#[doc = "Field `EXE_ONLY2_START` reader - Exe only 2 area start offset"]
pub type ExeOnly2StartR = crate::FieldReader;
#[doc = "Field `EXE_ONLY2_END` reader - Exe only 2 area end offset"]
pub type ExeOnly2EndR = crate::FieldReader;
#[doc = "Field `EXE_ONLY_KEEP` reader - Keep exe only area when debug level changes"]
pub type ExeOnlyKeepR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Exe only 1 area start offset"]
    #[inline(always)]
    pub fn exe_only1_start(&self) -> ExeOnly1StartR {
        ExeOnly1StartR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Exe only 1 area end offset"]
    #[inline(always)]
    pub fn exe_only1_end(&self) -> ExeOnly1EndR {
        ExeOnly1EndR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Exe only 2 area start offset"]
    #[inline(always)]
    pub fn exe_only2_start(&self) -> ExeOnly2StartR {
        ExeOnly2StartR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Exe only 2 area end offset"]
    #[inline(always)]
    pub fn exe_only2_end(&self) -> ExeOnly2EndR {
        ExeOnly2EndR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Keep exe only area when debug level changes"]
    #[inline(always)]
    pub fn exe_only_keep(&self) -> ExeOnlyKeepR {
        ExeOnlyKeepR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "option exe-only bytes register\n\nYou can [`read`](crate::Reg::read) this register and get [`option_eo_bytes::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptionEoBytesSpec;
impl crate::RegisterSpec for OptionEoBytesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`option_eo_bytes::R`](R) reader structure"]
impl crate::Readable for OptionEoBytesSpec {}
