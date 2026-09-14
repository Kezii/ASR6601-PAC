#[doc = "Register `LCR` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LcrSpec>;
#[doc = "Field `SLV` reader - phase decrementer load value for standard mode scl in master mode"]
pub type SlvR = crate::FieldReader<u16>;
#[doc = "Field `SLV` writer - phase decrementer load value for standard mode scl in master mode"]
pub type SlvW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FLV` reader - phase decrementer load value for fast mode scl in master mode"]
pub type FlvR = crate::FieldReader<u16>;
#[doc = "Field `FLV` writer - phase decrementer load value for fast mode scl in master mode"]
pub type FlvW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - phase decrementer load value for standard mode scl in master mode"]
    #[inline(always)]
    pub fn slv(&self) -> SlvR {
        SlvR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:17 - phase decrementer load value for fast mode scl in master mode"]
    #[inline(always)]
    pub fn flv(&self) -> FlvR {
        FlvR::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - phase decrementer load value for standard mode scl in master mode"]
    #[inline(always)]
    pub fn slv(&mut self) -> SlvW<'_, LcrSpec> {
        SlvW::new(self, 0)
    }
    #[doc = "Bits 9:17 - phase decrementer load value for fast mode scl in master mode"]
    #[inline(always)]
    pub fn flv(&mut self) -> FlvW<'_, LcrSpec> {
        FlvW::new(self, 9)
    }
}
#[doc = "load count register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
}
