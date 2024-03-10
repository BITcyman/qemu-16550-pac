#[doc = "Register `dlh` reader"]
pub type R = crate::R<DlhSpec>;
#[doc = "Register `dlh` writer"]
pub type W = crate::W<DlhSpec>;
#[doc = "Field `dlh` reader - "]
pub type DlhR = crate::FieldReader;
#[doc = "Field `dlh` writer - "]
pub type DlhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dlh(&self) -> DlhR {
        DlhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn dlh(&mut self) -> DlhW<DlhSpec> {
        DlhW::new(self, 0)
    }
}
#[doc = "UART Divisor Latch High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlhSpec;
impl crate::RegisterSpec for DlhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dlh::R`](R) reader structure"]
impl crate::Readable for DlhSpec {}
#[doc = "`write(|w| ..)` method takes [`dlh::W`](W) writer structure"]
impl crate::Writable for DlhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets dlh to value 0"]
impl crate::Resettable for DlhSpec {
    const RESET_VALUE: u8 = 0;
}
