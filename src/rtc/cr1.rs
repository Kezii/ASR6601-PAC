#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `WAKEUP2_SR_INT_EN` reader - Wakeup2 sr interrupt enable"]
pub type Wakeup2SrIntEnR = crate::BitReader;
#[doc = "Field `WAKEUP2_SR_INT_EN` writer - Wakeup2 sr interrupt enable"]
pub type Wakeup2SrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP1_SR_INT_EN` reader - Wakeup1 sr interrupt enable"]
pub type Wakeup1SrIntEnR = crate::BitReader;
#[doc = "Field `WAKEUP1_SR_INT_EN` writer - Wakeup1 sr interrupt enable"]
pub type Wakeup1SrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP0_SR_INT_EN` reader - Wakeup0 sr interrupt enable"]
pub type Wakeup0SrIntEnR = crate::BitReader;
#[doc = "Field `WAKEUP0_SR_INT_EN` writer - Wakeup0 sr interrupt enable"]
pub type Wakeup0SrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPER_SR_INT_EN` reader - Tamper sr interrupt enable"]
pub type TamperSrIntEnR = crate::BitReader;
#[doc = "Field `TAMPER_SR_INT_EN` writer - Tamper sr interrupt enable"]
pub type TamperSrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CYC_SR_INT_EN` reader - Cyc sr interrupt enable"]
pub type CycSrIntEnR = crate::BitReader;
#[doc = "Field `CYC_SR_INT_EN` writer - Cyc sr interrupt enable"]
pub type CycSrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM1_SR_INT_EN` reader - Alarm1 sr interrupt enable"]
pub type Alarm1SrIntEnR = crate::BitReader;
#[doc = "Field `ALARM1_SR_INT_EN` writer - Alarm1 sr interrupt enable"]
pub type Alarm1SrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM0_SR_INT_EN` reader - Alarm0 sr interrupt enable"]
pub type Alarm0SrIntEnR = crate::BitReader;
#[doc = "Field `ALARM0_SR_INT_EN` writer - Alarm0 sr interrupt enable"]
pub type Alarm0SrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECOND_SR_INT_EN` reader - Second sr interrupt enable"]
pub type SecondSrIntEnR = crate::BitReader;
#[doc = "Field `SECOND_SR_INT_EN` writer - Second sr interrupt enable"]
pub type SecondSrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup2 sr interrupt enable"]
    #[inline(always)]
    pub fn wakeup2_sr_int_en(&self) -> Wakeup2SrIntEnR {
        Wakeup2SrIntEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup1 sr interrupt enable"]
    #[inline(always)]
    pub fn wakeup1_sr_int_en(&self) -> Wakeup1SrIntEnR {
        Wakeup1SrIntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup0 sr interrupt enable"]
    #[inline(always)]
    pub fn wakeup0_sr_int_en(&self) -> Wakeup0SrIntEnR {
        Wakeup0SrIntEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper sr interrupt enable"]
    #[inline(always)]
    pub fn tamper_sr_int_en(&self) -> TamperSrIntEnR {
        TamperSrIntEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cyc sr interrupt enable"]
    #[inline(always)]
    pub fn cyc_sr_int_en(&self) -> CycSrIntEnR {
        CycSrIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alarm1 sr interrupt enable"]
    #[inline(always)]
    pub fn alarm1_sr_int_en(&self) -> Alarm1SrIntEnR {
        Alarm1SrIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alarm0 sr interrupt enable"]
    #[inline(always)]
    pub fn alarm0_sr_int_en(&self) -> Alarm0SrIntEnR {
        Alarm0SrIntEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Second sr interrupt enable"]
    #[inline(always)]
    pub fn second_sr_int_en(&self) -> SecondSrIntEnR {
        SecondSrIntEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup2 sr interrupt enable"]
    #[inline(always)]
    pub fn wakeup2_sr_int_en(&mut self) -> Wakeup2SrIntEnW<'_, Cr1Spec> {
        Wakeup2SrIntEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup1 sr interrupt enable"]
    #[inline(always)]
    pub fn wakeup1_sr_int_en(&mut self) -> Wakeup1SrIntEnW<'_, Cr1Spec> {
        Wakeup1SrIntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup0 sr interrupt enable"]
    #[inline(always)]
    pub fn wakeup0_sr_int_en(&mut self) -> Wakeup0SrIntEnW<'_, Cr1Spec> {
        Wakeup0SrIntEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper sr interrupt enable"]
    #[inline(always)]
    pub fn tamper_sr_int_en(&mut self) -> TamperSrIntEnW<'_, Cr1Spec> {
        TamperSrIntEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Cyc sr interrupt enable"]
    #[inline(always)]
    pub fn cyc_sr_int_en(&mut self) -> CycSrIntEnW<'_, Cr1Spec> {
        CycSrIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Alarm1 sr interrupt enable"]
    #[inline(always)]
    pub fn alarm1_sr_int_en(&mut self) -> Alarm1SrIntEnW<'_, Cr1Spec> {
        Alarm1SrIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Alarm0 sr interrupt enable"]
    #[inline(always)]
    pub fn alarm0_sr_int_en(&mut self) -> Alarm0SrIntEnW<'_, Cr1Spec> {
        Alarm0SrIntEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Second sr interrupt enable"]
    #[inline(always)]
    pub fn second_sr_int_en(&mut self) -> SecondSrIntEnW<'_, Cr1Spec> {
        SecondSrIntEnW::new(self, 7)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
