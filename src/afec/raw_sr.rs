#[doc = "Register `RAW_SR` reader"]
pub type R = crate::R<RawSrSpec>;
#[doc = "Field `RCO24M_READY` reader - Rco24m ready"]
pub type Rco24mReadyR = crate::BitReader;
#[doc = "Field `PLL_UNLOCK` reader - Pll unlock"]
pub type PllUnlockR = crate::BitReader;
#[doc = "Field `RCO4M_READY` reader - Rco4m ready"]
pub type Rco4mReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Rco24m ready"]
    #[inline(always)]
    pub fn rco24m_ready(&self) -> Rco24mReadyR {
        Rco24mReadyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 30 - Pll unlock"]
    #[inline(always)]
    pub fn pll_unlock(&self) -> PllUnlockR {
        PllUnlockR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Rco4m ready"]
    #[inline(always)]
    pub fn rco4m_ready(&self) -> Rco4mReadyR {
        Rco4mReadyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawSrSpec;
impl crate::RegisterSpec for RawSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_sr::R`](R) reader structure"]
impl crate::Readable for RawSrSpec {}
