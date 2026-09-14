#[doc = "Register `SAR` reader"]
pub type R = crate::R<SarSpec>;
#[doc = "Register `SAR` writer"]
pub type W = crate::W<SarSpec>;
#[doc = "Field `SLAVE_ADDRESS` reader - i2c slave address"]
pub type SlaveAddressR = crate::FieldReader;
#[doc = "Field `SLAVE_ADDRESS` writer - i2c slave address"]
pub type SlaveAddressW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - i2c slave address"]
    #[inline(always)]
    pub fn slave_address(&self) -> SlaveAddressR {
        SlaveAddressR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - i2c slave address"]
    #[inline(always)]
    pub fn slave_address(&mut self) -> SlaveAddressW<'_, SarSpec> {
        SlaveAddressW::new(self, 0)
    }
}
#[doc = "slave address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SarSpec;
impl crate::RegisterSpec for SarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SarSpec {}
#[doc = "`write(|w| ..)` method takes [`sar::W`](W) writer structure"]
impl crate::Writable for SarSpec {
    type Safety = crate::Unsafe;
}
