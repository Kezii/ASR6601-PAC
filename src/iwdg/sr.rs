#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `WRITE_CR_DONE` reader - Write cr done"]
pub type WriteCrDoneR = crate::BitReader;
#[doc = "Field `MAX_SET_DONE` reader - Max set done"]
pub type MaxSetDoneR = crate::BitReader;
#[doc = "Field `WIN_SET_DONE` reader - Win set done"]
pub type WinSetDoneR = crate::BitReader;
#[doc = "Field `WRITE_SR2_DONE` reader - Write sr2 done"]
pub type WriteSr2DoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write cr done"]
    #[inline(always)]
    pub fn write_cr_done(&self) -> WriteCrDoneR {
        WriteCrDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max set done"]
    #[inline(always)]
    pub fn max_set_done(&self) -> MaxSetDoneR {
        MaxSetDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Win set done"]
    #[inline(always)]
    pub fn win_set_done(&self) -> WinSetDoneR {
        WinSetDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write sr2 done"]
    #[inline(always)]
    pub fn write_sr2_done(&self) -> WriteSr2DoneR {
        WriteSr2DoneR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
