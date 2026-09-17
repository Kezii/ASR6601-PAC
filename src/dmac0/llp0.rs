#[doc = "Register `LLP0` reader"]
pub type R = crate::R<Llp0Spec>;
#[doc = "Register `LLP0` writer"]
pub type W = crate::W<Llp0Spec>;
#[doc = "Field `LOC` reader - The first address of the next LLI chain table"]
pub type LocR = crate::FieldReader<u32>;
#[doc = "Field `LOC` writer - The first address of the next LLI chain table"]
pub type LocW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The first address of the next LLI chain table"]
    #[inline(always)]
    pub fn loc(&self) -> LocR {
        LocR::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The first address of the next LLI chain table"]
    #[inline(always)]
    pub fn loc(&mut self) -> LocW<'_, Llp0Spec> {
        LocW::new(self, 0)
    }
}
#[doc = "linked list pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`llp0::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Llp0Spec;
impl crate::RegisterSpec for Llp0Spec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`llp0::R`](R) reader structure"]
impl crate::Readable for Llp0Spec {}
#[doc = "`write(|w| ..)` method takes [`llp0::W`](W) writer structure"]
impl crate::Writable for Llp0Spec {
    type Safety = crate::Unsafe;
}
