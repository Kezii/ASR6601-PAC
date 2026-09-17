#[doc = "Register `PPMADJUST` reader"]
pub type R = crate::R<PpmadjustSpec>;
#[doc = "Register `PPMADJUST` writer"]
pub type W = crate::W<PpmadjustSpec>;
#[doc = "Field `PPMADJUST_VALUE` reader - Ppm adjust value"]
pub type PpmadjustValueR = crate::FieldReader<u16>;
#[doc = "Field `PPMADJUST_VALUE` writer - Ppm adjust value"]
pub type PpmadjustValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Ppm adjust value"]
    #[inline(always)]
    pub fn ppmadjust_value(&self) -> PpmadjustValueR {
        PpmadjustValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ppm adjust value"]
    #[inline(always)]
    pub fn ppmadjust_value(&mut self) -> PpmadjustValueW<'_, PpmadjustSpec> {
        PpmadjustValueW::new(self, 0)
    }
}
#[doc = "ppm adjust value\n\nYou can [`read`](crate::Reg::read) this register and get [`ppmadjust::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppmadjust::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpmadjustSpec;
impl crate::RegisterSpec for PpmadjustSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppmadjust::R`](R) reader structure"]
impl crate::Readable for PpmadjustSpec {}
#[doc = "`write(|w| ..)` method takes [`ppmadjust::W`](W) writer structure"]
impl crate::Writable for PpmadjustSpec {
    type Safety = crate::Unsafe;
}
