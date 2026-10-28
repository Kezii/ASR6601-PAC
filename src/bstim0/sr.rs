#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `UIF` reader - Update interrupt flag; write 0 to clear"]
pub type UifR = crate::BitReader;
#[doc = "Field `UIF` writer - Update interrupt flag; write 0 to clear"]
pub type UifW<'a, REG> = crate::BitWriter0C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag; write 0 to clear"]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag; write 0 to clear"]
    #[inline(always)]
    pub fn uif(&mut self) -> UifW<'_, SrSpec> {
        UifW::new(self, 0)
    }
}
#[doc = "status register; UIF is cleared by writing 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
