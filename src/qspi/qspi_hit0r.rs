#[doc = "Register `QSPI_HIT0R` reader"]
pub type R = crate::R<QspiHit0rSpec>;
#[doc = "Register `QSPI_HIT0R` writer"]
pub type W = crate::W<QspiHit0rSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "hit0 times accumulator register\n\nYou can [`read`](crate::Reg::read) this register and get [`qspi_hit0r::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qspi_hit0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QspiHit0rSpec;
impl crate::RegisterSpec for QspiHit0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qspi_hit0r::R`](R) reader structure"]
impl crate::Readable for QspiHit0rSpec {}
#[doc = "`write(|w| ..)` method takes [`qspi_hit0r::W`](W) writer structure"]
impl crate::Writable for QspiHit0rSpec {
    type Safety = crate::Unsafe;
}
