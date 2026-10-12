#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `CMPMIE` reader - Compare match interrupt enable"]
pub type CmpmieR = crate::BitReader;
#[doc = "Field `CMPMIE` writer - Compare match interrupt enable"]
pub type CmpmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARRMIE` reader - Autoreload match interrupt enable"]
pub type ArrmieR = crate::BitReader;
#[doc = "Field `ARRMIE` writer - Autoreload match interrupt enable"]
pub type ArrmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTRIGIE` reader - External trigger interrupt enable"]
pub type ExttrigieR = crate::BitReader;
#[doc = "Field `EXTTRIGIE` writer - External trigger interrupt enable"]
pub type ExttrigieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPOKIE` reader - Compare register update ok interrupt enable"]
pub type CmpokieR = crate::BitReader;
#[doc = "Field `CMPOKIE` writer - Compare register update ok interrupt enable"]
pub type CmpokieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARROKIE` reader - Autoreload register update ok interrupt enable"]
pub type ArrokieR = crate::BitReader;
#[doc = "Field `ARROKIE` writer - Autoreload register update ok interrupt enable"]
pub type ArrokieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPIE` reader - Direction change to up interrupt enable"]
pub type UpieR = crate::BitReader;
#[doc = "Field `UPIE` writer - Direction change to up interrupt enable"]
pub type UpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNIE` reader - Direction change to down interrupt enable"]
pub type DownieR = crate::BitReader;
#[doc = "Field `DOWNIE` writer - Direction change to down interrupt enable"]
pub type DownieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare match interrupt enable"]
    #[inline(always)]
    pub fn cmpmie(&self) -> CmpmieR {
        CmpmieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Autoreload match interrupt enable"]
    #[inline(always)]
    pub fn arrmie(&self) -> ArrmieR {
        ArrmieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External trigger interrupt enable"]
    #[inline(always)]
    pub fn exttrigie(&self) -> ExttrigieR {
        ExttrigieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare register update ok interrupt enable"]
    #[inline(always)]
    pub fn cmpokie(&self) -> CmpokieR {
        CmpokieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoreload register update ok interrupt enable"]
    #[inline(always)]
    pub fn arrokie(&self) -> ArrokieR {
        ArrokieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction change to up interrupt enable"]
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Direction change to down interrupt enable"]
    #[inline(always)]
    pub fn downie(&self) -> DownieR {
        DownieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare match interrupt enable"]
    #[inline(always)]
    pub fn cmpmie(&mut self) -> CmpmieW<'_, IerSpec> {
        CmpmieW::new(self, 0)
    }
    #[doc = "Bit 1 - Autoreload match interrupt enable"]
    #[inline(always)]
    pub fn arrmie(&mut self) -> ArrmieW<'_, IerSpec> {
        ArrmieW::new(self, 1)
    }
    #[doc = "Bit 2 - External trigger interrupt enable"]
    #[inline(always)]
    pub fn exttrigie(&mut self) -> ExttrigieW<'_, IerSpec> {
        ExttrigieW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare register update ok interrupt enable"]
    #[inline(always)]
    pub fn cmpokie(&mut self) -> CmpokieW<'_, IerSpec> {
        CmpokieW::new(self, 3)
    }
    #[doc = "Bit 4 - Autoreload register update ok interrupt enable"]
    #[inline(always)]
    pub fn arrokie(&mut self) -> ArrokieW<'_, IerSpec> {
        ArrokieW::new(self, 4)
    }
    #[doc = "Bit 5 - Direction change to up interrupt enable"]
    #[inline(always)]
    pub fn upie(&mut self) -> UpieW<'_, IerSpec> {
        UpieW::new(self, 5)
    }
    #[doc = "Bit 6 - Direction change to down interrupt enable"]
    #[inline(always)]
    pub fn downie(&mut self) -> DownieW<'_, IerSpec> {
        DownieW::new(self, 6)
    }
}
#[doc = "LPTIM interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
