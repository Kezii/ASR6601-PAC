#[doc = "Register `SSP_ICR` reader"]
pub type R = crate::R<SspIcrSpec>;
#[doc = "Register `SSP_ICR` writer"]
pub type W = crate::W<SspIcrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ssp interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssp_icr::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssp_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspIcrSpec;
impl crate::RegisterSpec for SspIcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssp_icr::R`](R) reader structure"]
impl crate::Readable for SspIcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_icr::W`](W) writer structure"]
impl crate::Writable for SspIcrSpec {
    type Safety = crate::Unsafe;
}
