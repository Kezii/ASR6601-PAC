#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `WAKEUP2_SR` reader - Wakeup2 status"]
pub type Wakeup2SrR = crate::BitReader;
#[doc = "Field `WAKEUP2_SR` writer - Wakeup2 status"]
pub type Wakeup2SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP1_SR` reader - Wakeup1 status"]
pub type Wakeup1SrR = crate::BitReader;
#[doc = "Field `WAKEUP1_SR` writer - Wakeup1 status"]
pub type Wakeup1SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP0_SR` reader - Wakeup0 status"]
pub type Wakeup0SrR = crate::BitReader;
#[doc = "Field `WAKEUP0_SR` writer - Wakeup0 status"]
pub type Wakeup0SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPER_SR` reader - Tamper status"]
pub type TamperSrR = crate::BitReader;
#[doc = "Field `TAMPER_SR` writer - Tamper status"]
pub type TamperSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYC_SR` reader - Cyc status"]
pub type CycSrR = crate::BitReader;
#[doc = "Field `CYC_SR` writer - Cyc status"]
pub type CycSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM1_SR` reader - Alarm1 status"]
pub type Alarm1SrR = crate::BitReader;
#[doc = "Field `ALARM1_SR` writer - Alarm1 status"]
pub type Alarm1SrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM0_SR` reader - Alarm0 status"]
pub type Alarm0SrR = crate::BitReader;
#[doc = "Field `ALARM0_SR` writer - Alarm0 status"]
pub type Alarm0SrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup2 status"]
    #[inline(always)]
    pub fn wakeup2_sr(&self) -> Wakeup2SrR {
        Wakeup2SrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup1 status"]
    #[inline(always)]
    pub fn wakeup1_sr(&self) -> Wakeup1SrR {
        Wakeup1SrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup0 status"]
    #[inline(always)]
    pub fn wakeup0_sr(&self) -> Wakeup0SrR {
        Wakeup0SrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper status"]
    #[inline(always)]
    pub fn tamper_sr(&self) -> TamperSrR {
        TamperSrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cyc status"]
    #[inline(always)]
    pub fn cyc_sr(&self) -> CycSrR {
        CycSrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alarm1 status"]
    #[inline(always)]
    pub fn alarm1_sr(&self) -> Alarm1SrR {
        Alarm1SrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alarm0 status"]
    #[inline(always)]
    pub fn alarm0_sr(&self) -> Alarm0SrR {
        Alarm0SrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup2 status"]
    #[inline(always)]
    pub fn wakeup2_sr(&mut self) -> Wakeup2SrW<'_, SrSpec> {
        Wakeup2SrW::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup1 status"]
    #[inline(always)]
    pub fn wakeup1_sr(&mut self) -> Wakeup1SrW<'_, SrSpec> {
        Wakeup1SrW::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup0 status"]
    #[inline(always)]
    pub fn wakeup0_sr(&mut self) -> Wakeup0SrW<'_, SrSpec> {
        Wakeup0SrW::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper status"]
    #[inline(always)]
    pub fn tamper_sr(&mut self) -> TamperSrW<'_, SrSpec> {
        TamperSrW::new(self, 3)
    }
    #[doc = "Bit 4 - Cyc status"]
    #[inline(always)]
    pub fn cyc_sr(&mut self) -> CycSrW<'_, SrSpec> {
        CycSrW::new(self, 4)
    }
    #[doc = "Bit 5 - Alarm1 status"]
    #[inline(always)]
    pub fn alarm1_sr(&mut self) -> Alarm1SrW<'_, SrSpec> {
        Alarm1SrW::new(self, 5)
    }
    #[doc = "Bit 6 - Alarm0 status"]
    #[inline(always)]
    pub fn alarm0_sr(&mut self) -> Alarm0SrW<'_, SrSpec> {
        Alarm0SrW::new(self, 6)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
