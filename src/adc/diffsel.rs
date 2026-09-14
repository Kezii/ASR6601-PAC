#[doc = "Register `DIFFSEL` reader"]
pub type R = crate::R<DiffselSpec>;
#[doc = "Register `DIFFSEL` writer"]
pub type W = crate::W<DiffselSpec>;
#[doc = "Field `SEL0` reader - differential or single-ended mode selection for channels 1 to 8"]
pub type Sel0R = crate::FieldReader;
#[doc = "Field `SEL0` writer - differential or single-ended mode selection for channels 1 to 8"]
pub type Sel0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEL1` reader - channels 9 to 15 internal channels, single-ended only"]
pub type Sel1R = crate::FieldReader;
#[doc = "Field `SEL1` writer - channels 9 to 15 internal channels, single-ended only"]
pub type Sel1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:8 - differential or single-ended mode selection for channels 1 to 8"]
    #[inline(always)]
    pub fn sel0(&self) -> Sel0R {
        Sel0R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bits 9:15 - channels 9 to 15 internal channels, single-ended only"]
    #[inline(always)]
    pub fn sel1(&self) -> Sel1R {
        Sel1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:8 - differential or single-ended mode selection for channels 1 to 8"]
    #[inline(always)]
    pub fn sel0(&mut self) -> Sel0W<'_, DiffselSpec> {
        Sel0W::new(self, 1)
    }
    #[doc = "Bits 9:15 - channels 9 to 15 internal channels, single-ended only"]
    #[inline(always)]
    pub fn sel1(&mut self) -> Sel1W<'_, DiffselSpec> {
        Sel1W::new(self, 9)
    }
}
#[doc = "difference register\n\nYou can [`read`](crate::Reg::read) this register and get [`diffsel::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diffsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DiffselSpec;
impl crate::RegisterSpec for DiffselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diffsel::R`](R) reader structure"]
impl crate::Readable for DiffselSpec {}
#[doc = "`write(|w| ..)` method takes [`diffsel::W`](W) writer structure"]
impl crate::Writable for DiffselSpec {
    type Safety = crate::Unsafe;
}
